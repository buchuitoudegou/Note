#include <atomic>

class RWLatch {
public:
  RWLatch();
  void RLock();
  void WLock();
  void RUnLock();
  void WUnLock();
private:
  std::atomic<bool> _writer_entered;
  std::atomic<int> _reader_counter;
};