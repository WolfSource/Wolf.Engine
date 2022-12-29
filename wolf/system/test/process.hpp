/*
    Project: Wolf Engine. Copyright © 2014-2022 Pooya Eimandar
    https://github.com/WolfEngine/WolfEngine
*/

#ifdef WOLF_TEST

#pragma once

#include <wolf.hpp>
#include <system/w_leak_detector.hpp>
#include <boost/test/included/unit_test.hpp>

#include <system/w_process.hpp>

BOOST_AUTO_TEST_CASE(process_current_path_test) {
  const wolf::system::w_leak_detector _detector = {};
  using w_process = wolf::system::w_process;

  std::cout << "process_current_path_test is running" << std::endl;

  const auto _path = w_process::current_path();
  BOOST_REQUIRE(_path.has_error() == false);
  BOOST_REQUIRE(_path.value().empty() == false);

  const auto _exe_path = w_process::current_exe_path();
  BOOST_REQUIRE(_exe_path.has_error() == false);
  BOOST_REQUIRE(_exe_path.value().empty() == false);

  std::cout << "process_current_path_test just done" << std::endl;
}

#endif // WOLF_TESTS