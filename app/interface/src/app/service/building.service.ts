import { Injectable } from '@angular/core';
import { ConfigService } from './config.service';
import { BehaviorSubject } from 'rxjs';
import { v4 as uuidv4 } from 'uuid';


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

@Injectable({
  providedIn: 'root'
})
export class BuildingService {

  buildings: Building[] = [];
  buildings$ = new BehaviorSubject<Building[]>([]);


  constructor(private config: ConfigService) {  }


  create_buildings(format: number[]) {
    if (this.buildings.length > 0) {
      this.buildings$.next(this.buildings)
      return this.buildings
    }

    let buildings = []
    let { updatet_fomat, idx, max_row } = this.extend_format(format)

    let shift_a = 40 * this.config.scale
    let shift_b = 0

    for (let row = 0; row < updatet_fomat.length; row++) {
      let row_shift = max_row - updatet_fomat[row];

      if (row == idx + 1) {
        shift_a = 0
        shift_b = 40 * this.config.scale
      }

      let upper = true

      for (let i = 0; i < (updatet_fomat[row] * 2) + 1; i++) {
        buildings.push(this.construct_building(row, row_shift, i, (upper ? shift_a : shift_b)))
        upper = !upper;
      }
    }
    this.buildings = buildings;
    this.buildings$.next(this.buildings)
    return this.buildings
  }


  construct_building(row: number, row_shift: number, column: number, shift: number): Building {
    return {
      idx: uuidv4(),
      x: (row_shift + column) * (this.config.img_with / 2) * this.config.scale + this.config.padding_x | 0,
      y: row * 152 * this.config.scale + this.config.padding_y + shift | 0,
      color: "#aaa",
      type: "cottage", // cottage, location_city
      x_flatten: column + row_shift,
      y_flatten: row,
      shift: shift != 0
    }
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
