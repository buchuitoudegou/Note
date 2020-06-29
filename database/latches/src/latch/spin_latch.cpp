#include "latch/spin_latch.h"

bool SpinLatch::locked = true;
bool SpinLatch::unlocked = false;

SpinLatch::SpinLatch(): _is_lock(unlocked) {}

void SpinLatch::Lock() {
  while (atomic_flag_test_and_set(&_is_lock));
}

void SpinLatch::UnLock() {
  _is_lock.clear();
}