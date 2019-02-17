import { Injectable } from '@angular/core';
import { Observable, Subject, Subscription } from 'rxjs';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';

import { FrameMessage } from './frame-message';

@Injectable({
  providedIn: 'root'
})
export class VideoService {
  private subject: WebSocketSubject<Object>;
  private subscriberCount = 0;
  private subscription: Subscription;


  constructor() { }

  public connect(url?: string): WebSocketSubject<Object> {
    if (!url) {
      let loc = window.location;
      if (loc.protocol === "https:") {
        url = "wss:";
      } else {
        url = "ws:";
      }
      url += "//" + loc.host;
      url += loc.pathname + "v1/ws_json";
      //      url = 'ws://localhost:3000/v1/ws_json';
    }
    if (!this.subject) {
      this.subject = webSocket(url);
    }

    return this.subject;
  }

  private setupAcker() {
    if (this.subscriberCount == 0) {
      this.subscription = this.subject.subscribe(
        () => {
          this.subject.next(
            {
              command: 'ack',
              resolution: { type: 'HD' },
              cameraIds: [],
            }
          );
        },
        () => { },
        () => { }
      );
    }
  }

  private cleanupAcker() {
    if (this.subscriberCount == 0 && this.subscription) {
      this.subscription.unsubscribe();
    }
  }

  public getObservable(cameraId: number, resolution: string): Observable<FrameMessage> {
    let frameSub: WebSocketSubject<FrameMessage> = this.subject as unknown as WebSocketSubject<FrameMessage>;

    return frameSub.multiplex(
      () => {
        this.setupAcker();
        this.subscriberCount++;
        return {
          command: 'subscribe',
          resolution: { type: resolution },
          cameraIds: [cameraId],
        }
      },
      () => {
        this.subscriberCount--;
        this.cleanupAcker();
        return {
          command: 'unsubscribe',
          resolution: { type: resolution },
          cameraIds: [cameraId],
        };
      },
      (m) => {
        return m.cameraId === cameraId && m.resolution.type === resolution;
      }
    );
  }

  public getWriteSubject(): Subject<Object> {
    return this.subject;
  }
}
