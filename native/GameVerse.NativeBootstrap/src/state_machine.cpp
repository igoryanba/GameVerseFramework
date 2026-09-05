#include "gameverse/bootstrap.hpp"

namespace gameverse {

std::string_view StateName(BootstrapState state) noexcept {
  switch (state) {
    case BootstrapState::loaded: return "loaded";
    case BootstrapState::verified: return "verified";
    case BootstrapState::frontend_ready: return "frontend_ready";
    case BootstrapState::world_requested: return "world_requested";
    case BootstrapState::world_ready: return "world_ready";
    case BootstrapState::adapter_ready: return "adapter_ready";
    case BootstrapState::failed: return "failed";
    case BootstrapState::stopped: return "stopped";
  }
  return "failed";
}

bool StateMachine::Advance(BootstrapState next) noexcept {
  if (state_ == BootstrapState::failed || state_ == BootstrapState::stopped) return false;
  const auto current = static_cast<int>(state_);
  const auto requested = static_cast<int>(next);
  if (requested != current + 1 || next == BootstrapState::failed) return false;
  state_ = next;
  return true;
}

void StateMachine::Fail() noexcept { state_ = BootstrapState::failed; }

}  // namespace gameverse
