```js
class Observable {
  constructor(data) {
    this._store = data;
    this.observer = []
    if (typeof(data) == 'object') {
      for (let key in data) {
        Object.defineProperty(this, key, {
          get: () => {
            return this._store[key];
          },
          set: (newVal) => {
            this._store[key] = newVal;
            this.observer.forEach((obs) => this.publish(obs));
          }
        });
      }
    }
  }

  subscribe(observer) {
    this.observer.push(observer);
  }

  publish(observer) {
    const next = observer.next;
    const error = observer.error;
    try {  
      next(this);
    } catch (e) {
      if (error) {
        error(e);
      } else {
        throw `undefined error handler`;
      }
    }
    const complete = observer.complete;
    if (complete) {
      complete();
    }
  }
}

class mockHttpClient {
  get(url) {
    let httpResponse = new Observable({ status: 200, data: 'waiting...'});
    setTimeout(() => {
      httpResponse.status = Math.round(Math.random() * 200);
    }, 2000);
    setTimeout(() => {
      httpResponse.data = Date.now().toString();
    }, 3000);
    return httpResponse;
  }
}

class mockService {
  constructor() {
    this.http = new mockHttpClient();
  }
  getHttpTime(url) {
    return this.http.get(url);
  }
}

const mService = new mockService();

let clientTime = '';
let clientTimeObservable = mService.getHttpTime('').subscribe({
  next: (res) => { 
    console.log(`data changed.`); 
    clientTime = res._store; 
    console.log(res); 
  },
  error: (e) => console.log(e),
  complete: () => {}
});

```
