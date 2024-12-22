#pragma once

extern "C" void (*CallRustHandleDevicePoseUpdated)(int openVRID, void *pose);

#ifdef __cplusplus
extern "C"
{
#endif

    void *CppOpenVREntryPoint(const char *pInterfaceName, int *pReturnCode);

#ifdef __cplusplus
}
#endif
