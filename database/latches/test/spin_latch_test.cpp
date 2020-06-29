#include "latch/spin_latch.h"
#include <thread>
#include <assert.h>
#include <iostream>
// global counter
int counter = 0;

SpinLatch latch;

void test_equal(int test_val, int expected) {
  std::cout << test_val << " " << expected << std::endl;
  assert(test_val == expected);
}

void spin_latch_test() {
  std::thread t1([&]() {
    latch.Lock();
    counter = 11;
    test_equal(counter, 11);
    latch.UnLock();
  });
  std::thread t2([&]() {
    latch.Lock();
    counter = 22;
    test_equal(counter, 22);
    latch.UnLock();
  });
  std::thread t3([&] {
    latch.Lock();
    counter = 33;
    test_equal(counter, 33);
    latch.UnLock();
  });
  t2.join();
  t1.join();
  t3.join();
}

int main() {
  spin_latch_test();
}