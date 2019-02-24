```JS
// http demo
async function httpRequest() {
  const value = Math.random() * 100;
  if (value < 50) {
    throw `value is too small`;
  }
  return new Promise((res) => {
    setTimeout(() => {
      res(value);
    }, 1000);
  });
}
// define Observable class
class Observable {
  constructor(fn) {
    this.callback = fn;
  }
  subscribe({next, error, complete}) {
    const observer = {
      next: next,
      error: error,
      complete: complete
    };
    return this.callback(observer);
  }
};
// how to obtain and generate values or messages to publish.
async function subscriber(observer) {
  const { next, error, complete } = observer;
  try {
    const value = await httpRequest();
    next(value);
  } catch (err) {
    error(err);
  }
  complete();
  return {unsubscribe() { console.log('unsubscribe'); }};
}

const observableExample = new Observable(subscriber);
const subscription = observableExample.subscribe({
  next: value => console.log(`get a value: ${value}`),
  error: (err) => console.log(`error catch: ${err}`),
  complete: () => console.log(`finished`)
});

```