#include "gameverse/bootstrap.hpp"

#include <windows.h>

namespace {
DWORD WINAPI BootstrapThread(void* module) {
  gameverse::RunBootstrap(module);
  FreeLibraryAndExitThread(static_cast<HMODULE>(module), 0);
}
}  // namespace

BOOL WINAPI DllMain(HINSTANCE instance, DWORD reason, void*) {
  if (reason != DLL_PROCESS_ATTACH) return TRUE;
  DisableThreadLibraryCalls(instance);
  HANDLE thread = CreateThread(nullptr, 0, BootstrapThread, instance, 0, nullptr);
  if (!thread) return FALSE;
  CloseHandle(thread);
  return TRUE;
}
