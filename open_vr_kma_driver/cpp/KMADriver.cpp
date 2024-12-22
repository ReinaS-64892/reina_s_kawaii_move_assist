#pragma once
#include "bindings.h"
#include "KMADriver.h"
#include <openvr_driver.h>
#include "OVR-SC/InterfaceHookInjector.h"

vr::EVRInitError HookDriver::Init(vr::IVRDriverContext *pDriverContext)
{
    InjectHooks(pDriverContext);
    return vr::EVRInitError::VRInitError_None;
}

void *CppOpenVREntryPoint(const char *pInterfaceName, int *pReturnCode)
{
    if (std::strcmp(vr::IServerTrackedDeviceProvider_Version, pInterfaceName) == 0) // どうやら strcmp は 等しいときに 0 を返すらしい ...
    {
        *pReturnCode = vr::VRInitError_None;
        if (s_driver == nullptr)
        {
            s_driver = new HookDriver;
        }

        return s_driver;
    }

    *pReturnCode = vr::VRInitError_Init_InterfaceNotFound;
    return nullptr;
}
void (*CallRustHandleDevicePoseUpdated)(int openVRID, void *pose);

void HandleDevicePoseUpdated(uint32_t unWhichDevice, vr::DriverPose_t *newPose)
{
    CallRustHandleDevicePoseUpdated((int)unWhichDevice, (void *)newPose);
}
