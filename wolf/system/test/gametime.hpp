/*
    Project: Wolf Engine. Copyright © 2014-2022 Pooya Eimandar
    https://github.com/WolfEngine/WolfEngine
*/

#pragma once

#include <gtest.hpp>
#include <system/w_gametime.hpp>

TEST(gametime, fixed_time) {
  const wolf::system::w_leak_detector _detector = {};
  using w_gametime = wolf::system::w_gametime;

  std::once_flag _flag;
  bool _ticked = false;

  constexpr double _stop_after_secs = 3.0;
  constexpr double _target_elapsed_secs = 1.0 / 50.0; // 50 fps

  auto _gametime = w_gametime();
  _gametime.reset();
  _gametime.set_fixed_time_step(true);
  _gametime.set_target_elapsed_secs(_target_elapsed_secs);

  auto _total_seconds = _gametime.get_total_secs();
#ifdef __clang__
#pragma unroll
#endif
  while (_total_seconds < _stop_after_secs) {
    _gametime.tick([&]() { std::call_once(_flag, [&]() { _ticked = true; }); });

    _total_seconds = _gametime.get_total_secs();
  }

  EXPECT_EQ(_ticked, true);
}

TEST(gametime, unfixed_time) {
        const wolf::system::w_leak_detector _detector = {};

        constexpr double _stop_after_secs = 5.0;
        constexpr double _target_elapsed_secs = 1.0 / 50.0; // 50 fps

        using w_gametime = wolf::system::w_gametime;

        auto _gametime = w_gametime();
        _gametime.reset();
        _gametime.set_fixed_time_step(false);
        _gametime.set_target_elapsed_secs(_target_elapsed_secs);

        EXPECT_EQ(_gametime.get_fps(), 0);

        auto _total_seconds = _gametime.get_total_secs();
        while (_gametime.get_total_secs() < _stop_after_secs) {
                _gametime.tick();
                _total_seconds = _gametime.get_total_secs();
        }

        EXPECT_GT(_gametime.get_fps(), 50);
}
