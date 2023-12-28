import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { v4 as uuidv4 } from 'uuid';
import { ConfigService } from './config.service';



export interface Tile {
  idx: string,
  x: number;
  y: number;
  h: number;
  w: number,
  x_flatten: number;
  edit_mode: boolean;
  y_flatten: number;
  img_src: string;
  dice?: number;
}

@Injectable({
  providedIn: 'root'
})
export class TileService {

  tiles: Tile[] = [];
  tiles$ = new BehaviorSubject<Tile[]>([]);


  constructor(private config: ConfigService) {  }

  set_dice_value(idx: string, value: number) {
    this.tiles.filter(e => e.idx == idx)[0].dice = value
    this.tiles$.next(this.tiles)
  }

  set_terra(idx: string, value: string) {
    this.tiles.filter(e => e.idx == idx)[0].img_src = "assets/images/" + value
    this.tiles.filter(e => e.idx == idx)[0].edit_mode = false

    this.tiles$.next(this.tiles)
  }

  calcualate_size(format: number[]) {
    this.config.board_heigth.next(
      format.length * (this.config.img_heigt * this.config.scale)
      - ((format.length - 1) * this.config.img_heigt / 3.8 * this.config.scale)
      + (this.config.padding_y * 2)
    )

    this.config.board_width.next(
      (Math.max(...format) * 2) / 2 * this.config.img_with * this.config.scale + (this.config.padding_x * 2)
    )
  }

  create_tiles(format: number[]) {
    if (this.tiles.length > 0) {
      return this.tiles
    }

    for (let row = 0; row < format.length; row++) {
      let row_shift = Math.max(...format) - format[row];
      for (let column = 0; column < format[row]; column++) {
        this.tiles.push(this.constrct_tile(row, row_shift, column))
      }
    }
    this.tiles$.next(this.tiles)
    return this.tiles
  }

  constrct_tile(row: number, row_shift: number, column: number): Tile {
    return {
      idx: uuidv4(),
      x: (row_shift + column * 2) / 2 * this.config.img_with * this.config.scale + this.config.padding_x,
      y: row * (this.config.img_heigt * this.config.scale) - (row * this.config.img_heigt / 3.8 * this.config.scale) + this.config.padding_y,
      img_src: "assets/images/sechseck.svg",
      dice: undefined,
      edit_mode: false,
      x_flatten: column + row_shift,
      y_flatten: row,
      w: this.config.scale * this.config.img_with,
      h: this.config.scale * this.config.img_heigt
    }
  }
}
