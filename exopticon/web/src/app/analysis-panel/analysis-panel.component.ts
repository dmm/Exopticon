import { Component, Input, OnInit } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Observable } from "rxjs";
import { WsMessage } from "../frame-message";
import { SubscriptionSubject, VideoService } from "../video.service";

@Component({
  selector: "app-analysis-panel",
  templateUrl: "./analysis-panel.component.html",
  styleUrls: ["./analysis-panel.component.css"],
})
export class AnalysisPanelComponent implements OnInit {
  @Input() analysisEngineId: number;

  public videoSubject: SubscriptionSubject;
  public frameService?: Observable<WsMessage>;

  constructor(
    public route: ActivatedRoute,
    public videoService: VideoService
  ) {}

  ngOnInit() {
    this.videoService.connect();
    let id = this.route.snapshot.paramMap.get("id");
    this.videoSubject = {
      kind: "analysisEngine",
      analysisEngineId: parseInt(id, 10),
    };
    this.frameService = this.videoService.getObservable(this.videoSubject);
  }
}
