import { Component, Input, OnInit } from '@angular/core';
import { BoardService, Tile } from '../../../service/board.service';

@Component({
  selector: 'app-tile',
  standalone: true,
  imports: [],
  templateUrl: './tile.component.html',
  styleUrl: './tile.component.scss'
})
export class TileComponent implements OnInit {
  constructor(public baord: BoardService) { }

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
    this.baord.last_selected_tile.subscribe((e: string) => {
      if (this.tile.idx == e) {
        this.tile.edit_mode = true
      }else {
        this.tile.edit_mode = false
      }

    })
  }

  activate() {
    this.baord.tile_clicked(this.tile.idx)
  }

}
