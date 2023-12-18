import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable, Subject } from 'rxjs';
import { v4 as uuidv4 } from 'uuid';

export interface Tile {
  idx: string,
  x: number;
  y: number;
  h: number;
  w: number,
  x_flatten: number;
  y_flatten: number;
  img_src: string;
  dice?: number;
}

export interface Building {
  idx: string,

  x: number;
  y: number;
  x_flatten: number;
  y_flatten: number;
  color: string,
  type: string,
  shift: boolean,
}


export interface Street {
  idx: string,

  b1_x_flatten: number,
  b1_y_flatten: number,
  b2_x_flatten: number,
  b2_y_flatten: number,

  x: number;
  y: number;
  rotation: number;
  length: number;
  heigth: number;
  color: string
}

@Injectable({
  providedIn: 'root'
})
export class BoardService {

  format = new Subject<number[]>();
  buildings$ = new Subject<Building[]>();
  tiles$ = new Subject<Tile[]>();
  street$ = new Subject<Street[]>();


  board_heigth = new BehaviorSubject(0);
  board_width = new BehaviorSubject(0);


  img_with = 175;
  img_heigt = 203;
  padding_x = 50;
  padding_y = 50;

  scale = 0.7

  constructor() {
    this.create()
  }

  create() {
    this.format.subscribe((format) => {
      this.create_tiles(format)
      this.calcualate_size(format)
      let buildings = this.create_buildings(format)
      this.create_streets(format, buildings)
    })
  }


  calcualate_size(format: number[]) {
    this.board_heigth.next(
      format.length * (this.img_heigt * this.scale)
      - ((format.length - 1) * this.img_heigt / 3.8 * this.scale)
      + (this.padding_y * 2)
    )

    this.board_width.next(
      (Math.max(...format) * 2) / 2 * this.img_with * this.scale + (this.padding_x * 2)
    )
  }

  create_tiles(format: number[]) {
    let coordinats = [];
    for (let row = 0; row < format.length; row++) {
      let row_shift = Math.max(...format) - format[row];
      for (let column = 0; column < format[row]; column++) {
        coordinats.push(this.constrct_tile(row, row_shift, column))
      }
    }
    this.tiles$.next(coordinats)
    return coordinats
  }


  constrct_tile(row: number, row_shift: number, column: number): Tile {
    return {
      idx: uuidv4(),
      x: (row_shift + column * 2) / 2 * this.img_with * this.scale + this.padding_x,
      y: row * (this.img_heigt * this.scale) - (row * this.img_heigt / 3.8 * this.scale) + this.padding_y,
      img_src: "assets/images/sechseck.svg",
      dice: 6,
      x_flatten: column + row_shift,
      y_flatten: row,
      w: this.scale * this.img_with,
      h: this.scale * this.img_heigt
    }
  }

  create_buildings(format: number[]) {
    let buildings = []
    let { updatet_fomat, idx, max_row } = this.extend_format(format)

    let shift_a = 40 * this.scale
    let shift_b = 0

    for (let row = 0; row < updatet_fomat.length; row++) {
      let row_shift = max_row - updatet_fomat[row];

      if (row == idx + 1) {
        shift_a = 0
        shift_b = 40 * this.scale
      }

      let upper = true

      for (let i = 0; i < (updatet_fomat[row] * 2) + 1; i++) {
        buildings.push(this.construct_building(row, row_shift, i, (upper ? shift_a : shift_b)))
        upper = !upper;
      }
    }

    this.buildings$.next(buildings)
    return buildings
  }


  construct_building(row: number, row_shift: number, column: number, shift: number): Building {
    return {
      idx: uuidv4(),
      x: (row_shift + column) * (this.img_with / 2) * this.scale + this.padding_x | 0,
      y: row * 152 * this.scale + this.padding_y + shift | 0,
      color: "#aaa",
      type: "cottage", // cottage, location_city
      x_flatten: column + row_shift,
      y_flatten: row,
      shift: shift != 0
    }
  }


  create_streets(format: number[], buildings: Building[]) {
    let streets = []
    let { updatet_fomat, idx, max_row } = this.extend_format(format)


    for (let row = 0; row < updatet_fomat.length; row++) {
      let row_shift = (max_row - updatet_fomat[row]);

      for (let i = 0; i < (updatet_fomat[row] * 2); i++) {

        let a = buildings.filter((b) => b.y_flatten == row && b.x_flatten == row_shift + i)[0]
        let b = buildings.filter((b) => b.y_flatten == row && b.x_flatten == row_shift + i + 1)[0]

        let y_delta = Math.abs(a.y - b.y) / 2
        let y_root = Math.min(a.y, a.y)

        streets.push({
          idx: uuidv4(),
          b1_x_flatten: a.x_flatten,
          b1_y_flatten: a.y_flatten,
          b2_x_flatten: b.x_flatten,
          b2_y_flatten: b.y_flatten,
          color: "transparent",
          length: 30 * this.scale,
          heigth: 20 * this.scale,
          rotation: b.shift ? 30 : 150,
          x: Math.min(a.x, b.x) + Math.abs(a.x - b.x) / 2,
          y: b.shift ? y_root + y_delta : y_root - y_delta,

        })

      }
    }


    for (let row = 0; row < format.length; row++) {
      let row_shift = (max_row - format[row]);
      for (let i = 0; i < format[row] + 1; i++) {
        let c = row_shift + i * 2;

        let a = buildings.filter((b) => b.y_flatten == row && b.x_flatten == c)[0]
        let b = buildings.filter((b) => b.y_flatten == row + 1 && b.x_flatten == c)[0]


        streets.push({
          idx: uuidv4(),
          b1_x_flatten: a.x_flatten,
          b1_y_flatten: a.y_flatten,
          b2_x_flatten: b.x_flatten,
          b2_y_flatten: b.y_flatten,
          color: "transparent",
          length: 30 * this.scale,
          heigth: 20 * this.scale,
          rotation: 90,
          x: Math.min(a.x, b.x) + Math.abs(a.x - b.x) / 2,
          y: Math.min(a.y, b.y) + Math.abs(a.y - b.y) / 2,

        })

      }
    }

    this.street$.next(streets)

  }



  private extend_format(format: number[]): { updatet_fomat: number[], idx: number, max_row: number } {
    let max_row = Math.max(...format);
    let idx = format.indexOf(max_row);

    let updatet_fomat = format.slice(0, idx);
    updatet_fomat.push(max_row);
    updatet_fomat.push(...format.slice(idx))

    return { updatet_fomat, idx, max_row }
  }
}
