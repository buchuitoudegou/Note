#include "lock/rwlatch.h"
#include "latch/spin_latch.h"

RWLatch::RWLatch(): _writer_entered(false), _reader_counter(0) {}

void RWLatch::RLock() {
  
}

void RWLatch::WLock() {
  
}

void RWLatch::RUnLock() {
  
}

void RWLatch::WUnLock() {
  
}