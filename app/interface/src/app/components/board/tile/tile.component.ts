import { Component, Input } from '@angular/core';
import { Tile } from '../../../service/board.service';

@Component({
  selector: 'app-tile',
  standalone: true,
  imports: [],
  templateUrl: './tile.component.html',
  styleUrl: './tile.component.scss'
})
export class TileComponent {
  @Input() tile: Tile = {
    idx: "",
    x: 0,
    y: 0,
    h: 0,
    w: 0,
    x_flatten: 0,
    y_flatten: 0,
    img_src: "",
  };
}
