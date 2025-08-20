#include "HMD.h"

#include "Controller.h"
#include "Logger.h"
#include "Paths.h"
#include "PoseHistory.h"
#include "Settings.h"
#include "Utils.h"
#include "ViveTrackerProxy.h"
#include "bindings.h"
#include "platform/linux/CEncoder.h"

Hmd::Hmd()
    : TrackedDevice(
          HEAD_ID,
          Settings::Instance().m_TrackingRefOnly ? vr::TrackedDeviceClass_TrackingReference
                                                 : vr::TrackedDeviceClass_HMD
      )
    , m_baseComponentsInitialized(false)
    , m_streamComponentsInitialized(false) {
    Debug("Hmd::constructor");

    auto dummy_fov = FfiFov { -1.0, 1.0, 1.0, -1.0 };
    auto dummy_pose = FfiPose { { 0, 0, 0, 1 }, { 0, 0, 0 } };
    auto dummy_view_params = FfiViewParams { dummy_pose, dummy_fov };

    this->view_params[0] = dummy_view_params;
    this->view_params[1] = dummy_view_params;

    m_poseHistory = std::make_shared<PoseHistory>();

    if (Settings::Instance().m_enableViveTrackerProxy) {
        m_viveTrackerProxy = std::make_unique<ViveTrackerProxy>(*this);
        if (!vr::VRServerDriverHost()->TrackedDeviceAdded(
                m_viveTrackerProxy->GetSerialNumber(),
                vr::TrackedDeviceClass_GenericTracker,
                m_viveTrackerProxy.get()
            )) {
            Warn("Failed to register Vive tracker");
        }
    }
}

Hmd::~Hmd() {
    Debug("Hmd::destructor");

    if (m_encoder) {
        Debug("Hmd::~Hmd(): Stopping encoder...\n");
        m_encoder->Stop();
        m_encoder.reset();
    }
}

bool Hmd::activate() {
    Debug("Hmd::Activate");

    auto vr_properties = vr::VRProperties();

    SetOpenvrProps((void*)this, this->device_id);

    vr_properties->SetFloatProperty(
        this->prop_container,
        vr::Prop_DisplayFrequency_Float,
        static_cast<float>(Settings::Instance().m_refreshRate)
    );

    vr::VRDriverInput()->CreateBooleanComponent(this->prop_container, "/proximity", &m_proximity);

    // #ifdef _WIN32
    //     float originalIPD
    //         = vr::VRSettings()->GetFloat(vr::k_pch_SteamVR_Section, vr::k_pch_SteamVR_IPD_Float);
    //     vr::VRSettings()->SetFloat(vr::k_pch_SteamVR_Section, vr::k_pch_SteamVR_IPD_Float,
    //     0.063);
    // #endif
    // todo: might be needed for direct mode
    HmdMatrix_SetIdentity(&m_eyeToHeadLeft);
    HmdMatrix_SetIdentity(&m_eyeToHeadRight);

    // Disable async reprojection on Linux. Windows interface uses IVRDriverDirectModeComponent
    // which never applies reprojection
    // Also Disable async reprojection on vulkan
    // #ifndef _WIN32
    //     vr::VRSettings()->SetBool(
    //         vr::k_pch_SteamVR_Section,
    //         vr::k_pch_SteamVR_EnableLinuxVulkanAsync_Bool,
    //         Settings::Instance().m_enableLinuxVulkanAsyncCompute
    //     );
    //     vr::VRSettings()->SetBool(
    //         vr::k_pch_SteamVR_Section,
    //         vr::k_pch_SteamVR_DisableAsyncReprojection_Bool,
    //         !Settings::Instance().m_enableLinuxAsyncReprojection
    //     );
    // #endif
    // todo: might be needed for linux direct mode

    if (!m_baseComponentsInitialized) {
        m_baseComponentsInitialized = true;

        DriverReadyIdle(this->device_class == vr::TrackedDeviceClass_HMD);
    }

    if (this->device_class == vr::TrackedDeviceClass_HMD) {
        vr::VREvent_Data_t eventData;
        eventData.ipd = { 0.063 };
        vr::VRServerDriverHost()->VendorSpecificEvent(
            this->object_id, vr::VREvent_IpdChanged, eventData, 0
        );
    }

    return true;
}

void* Hmd::get_component(const char* component_name_and_version) {
    Debug("Hmd::GetComponent %s", component_name_and_version);

    // NB: "this" pointer needs to be statically cast to point to the correct vtable

    auto name_and_vers = std::string(component_name_and_version);
    if (name_and_vers == vr::IVRDisplayComponent_Version) {
        return (vr::IVRDisplayComponent*)this;
    }

    // #ifdef _WIN32
    //     if (name_and_vers == vr::IVRDriverDirectModeComponent_Version) {
    //         return m_directModeComponent.get();
    //     }
    // #endif
    // todo: might be needed for direct mode

    return nullptr;
}

void Hmd::OnPoseUpdated(uint64_t targetTimestampNs, FfiDeviceMotion motion) {
    Debug("Hmd::OnPoseUpdated");

    if (this->object_id == vr::k_unTrackedDeviceIndexInvalid) {
        return;
    }
    auto pose = vr::DriverPose_t {};
    pose.poseIsValid = true;
    pose.result = vr::TrackingResult_Running_OK;
    pose.deviceIsConnected = true;

    pose.qWorldFromDriverRotation = HmdQuaternion_Init(1, 0, 0, 0);
    pose.qDriverFromHeadRotation = HmdQuaternion_Init(1, 0, 0, 0);

    pose.qRotation = HmdQuaternion_Init(
        motion.pose.orientation.w,
        motion.pose.orientation.x,
        motion.pose.orientation.y,
        motion.pose.orientation.z
    );

    pose.vecPosition[0] = motion.pose.position[0];
    pose.vecPosition[1] = motion.pose.position[1];
    pose.vecPosition[2] = motion.pose.position[2];

    this->submit_pose(pose);

    m_poseHistory->OnPoseUpdated(targetTimestampNs, motion);

    if (m_viveTrackerProxy)
        m_viveTrackerProxy->update();

    // This has to be set after initialization is done, because something in vrcompositor is
    // setting it to 90Hz in the meantime
    if (!m_refreshRateSet && m_encoder && m_encoder->IsConnected()) {
        m_refreshRateSet = true;
        vr::VRProperties()->SetFloatProperty(
            this->prop_container,
            vr::Prop_DisplayFrequency_Float,
            static_cast<float>(Settings::Instance().m_refreshRate)
        );
    }
}

void Hmd::StartStreaming() {
    Debug("Hmd::StartStreaming");

    vr::VRDriverInput()->UpdateBooleanComponent(m_proximity, true, 0.0);

    if (m_streamComponentsInitialized) {
        return;
    }

    // Spin up a separate thread to handle the overlapped encoding/transmit step.
    if (this->device_class == vr::TrackedDeviceClass_HMD) {

        m_encoder = std::make_shared<CEncoder>(m_poseHistory);
        m_encoder->Start();
        m_encoder->OnStreamStart();
    }

    m_streamComponentsInitialized = true;
}

void Hmd::StopStreaming() {
    Debug("Hmd::StopStreaming");

    vr::VRDriverInput()->UpdateBooleanComponent(m_proximity, false, 0.0);
}

void Hmd::SetViewParams(const FfiViewParams params[2]) {
    Debug("Hmd::SetViewParams");

    this->view_params[0] = params[0];
    this->view_params[1] = params[1];

    // The OpenXR spec defines the HMD position as the midpoint
    // between the eyes, so conversion to this is handled by the
    // client.
    auto left_transform = pose_to_mat(params[0].pose);
    auto right_transform = pose_to_mat(params[1].pose);
    vr::VRServerDriverHost()->SetDisplayEyeToHead(object_id, left_transform, right_transform);

    auto left_proj = fov_to_tangents(params[0].fov);
    auto right_proj = fov_to_tangents(params[1].fov);
    vr::VRServerDriverHost()->SetDisplayProjectionRaw(object_id, left_proj, right_proj);

    // todo: check if this is still needed
    vr::VRServerDriverHost()->VendorSpecificEvent(
        object_id, vr::VREvent_LensDistortionChanged, {}, 0
    );
}

void Hmd::GetWindowBounds(int32_t* pnX, int32_t* pnY, uint32_t* pnWidth, uint32_t* pnHeight) {
    Debug(
        "Hmd::GetWindowBounds %dx%d - %dx%d\n",
        0,
        0,
        Settings::Instance().m_renderWidth,
        Settings::Instance().m_renderHeight
    );

    *pnX = 0;
    *pnY = 0;
    *pnWidth = Settings::Instance().m_renderWidth;
    *pnHeight = Settings::Instance().m_renderHeight;
}

bool Hmd::IsDisplayRealDisplay() {
    // #ifdef _WIN32
    //     return false;
    // #else
    //     return true;
    // #endif
    // todo: might be needed for linux direct mode
    return true;
}

void Hmd::GetRecommendedRenderTargetSize(uint32_t* pnWidth, uint32_t* pnHeight) {
    *pnWidth = Settings::Instance().m_recommendedTargetWidth / 2;
    *pnHeight = Settings::Instance().m_recommendedTargetHeight;
    Debug("Hmd::GetRecommendedRenderTargetSize %dx%d\n", *pnWidth, *pnHeight);
}

void Hmd::GetEyeOutputViewport(
    vr::EVREye eEye, uint32_t* pnX, uint32_t* pnY, uint32_t* pnWidth, uint32_t* pnHeight
) {
    *pnY = 0;
    *pnWidth = Settings::Instance().m_renderWidth / 2;
    *pnHeight = Settings::Instance().m_renderHeight;

    if (eEye == vr::Eye_Left) {
        *pnX = 0;
    } else {
        *pnX = Settings::Instance().m_renderWidth / 2;
    }

    Debug("Hmd::GetEyeOutputViewport Eye=%d %dx%d %dx%d\n", eEye, *pnX, *pnY, *pnWidth, *pnHeight);
}

void Hmd::GetProjectionRaw(vr::EVREye eye, float* left, float* right, float* top, float* bottom) {
    auto proj = fov_to_tangents(this->view_params[eye].fov);
    *left = proj.vTopLeft.v[0];
    *right = proj.vBottomRight.v[0];
    *top = proj.vTopLeft.v[1];
    *bottom = proj.vBottomRight.v[1];

    Debug("Hmd::GetProjectionRaw Eye=%d %f %f %f %f\n", eye, *left, *right, *top, *bottom);
}

vr::DistortionCoordinates_t Hmd::ComputeDistortion(vr::EVREye, float u, float v) {
    return { { u, v }, { u, v }, { u, v } };
}

bool Hmd::ComputeInverseDistortion(vr::HmdVector2_t*, vr::EVREye, uint32_t, float, float) {
    return false;
}
