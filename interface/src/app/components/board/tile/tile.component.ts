import { Component, Input, OnDestroy, OnInit } from '@angular/core';
import { Tile } from '../../../service/tile.service';
import { ActivatedRoute, Router } from '@angular/router';
import { BoardService } from '../../../service/board.service';


@Component({
  selector: 'app-tile',
  standalone: true,
  imports: [],
  templateUrl: './tile.component.html',
  styleUrl: './tile.component.scss'
})
export class TileComponent implements OnInit {
  constructor(public baord: BoardService, private _route: ActivatedRoute,
    private _router: Router) { }


  @Input() tile: Tile = {
    idx: "---",
    x: 0,
    y: 0,
    h: 0,
    w: 0,
    x_flatten: 0,
    y_flatten: 0,
    img_src: "",
    edit_mode:false
  };

  
  ngOnInit(): void {
    this._route.queryParams.subscribe((q) => {
      this.tile.edit_mode = q['tile'] == this.tile.idx
    })
  }

  activate() {
    this._router.navigate([], {
      relativeTo: this._route,
      queryParams: {
        tile: this.tile.idx
      },
      queryParamsHandling: 'merge',
      skipLocationChange: false
    });
  }

}
