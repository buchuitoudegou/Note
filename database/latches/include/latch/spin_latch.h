#include <atomic>

using namespace std;


class SpinLatch {
public:
  static bool unlocked;
  static bool locked;
  SpinLatch();
  void Lock();
  void UnLock();
private:
  atomic_flag _is_lock;
};