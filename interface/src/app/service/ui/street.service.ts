import { Injectable } from '@angular/core';
import { Building } from './building.service';
import { BehaviorSubject } from 'rxjs';
import { v4 as uuidv4 } from 'uuid';
import { ConfigService } from './config.service';

export interface Street {
  idx: string,
  buildings: Building[]
  x: number;
  y: number;
  rotation: number;
  length: number;
  heigth: number;
  color: string;
  edit_mode: boolean
}

@Injectable({
  providedIn: 'root'
})
export class StreetService {

  streets: Street[] = [];
  street$ = new BehaviorSubject<Street[]>([]);


  constructor(private config: ConfigService) { }

  create_streets(format: number[], buildings: Building[]) {
    if (this.streets.length > 0) {
      this.street$.next(this.streets)
      return
    }

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
          buildings: [a, b],
          edit_mode: false,
          color: "transparent",
          length: 30 * this.config.scale,
          heigth: 20 * this.config.scale,
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
          buildings: [a, b],
          edit_mode: false,
          color: "transparent",
          length: 30 * this.config.scale,
          heigth: 20 * this.config.scale,
          rotation: 90,
          x: Math.min(a.x, b.x) + Math.abs(a.x - b.x) / 2,
          y: Math.min(a.y, b.y) + Math.abs(a.y - b.y) / 2,

        })

      }
    }
    this.streets = streets;
    this.street$.next(this.streets)
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
