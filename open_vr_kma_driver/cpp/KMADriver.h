#pragma once
#include <openvr_driver.h>

class HookDriver : public vr::IServerTrackedDeviceProvider
{
public:
    vr::EVRInitError Init(vr::IVRDriverContext *pDriverContext) override;
    virtual void Cleanup() {}
    virtual const char *const *GetInterfaceVersions() { return vr::k_InterfaceVersions; }
    virtual void RunFrame() {}
    virtual bool ShouldBlockStandbyMode() { return false; }
    virtual void EnterStandby() {}
    virtual void LeaveStandby() {}
};

static HookDriver *s_driver = nullptr;

void HandleDevicePoseUpdated(uint32_t unWhichDevice, vr::DriverPose_t *newPose);
