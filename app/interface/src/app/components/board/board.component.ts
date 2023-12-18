import { Component, Input, OnInit, ɵɵpipeBind1 } from '@angular/core';
import {MatIconModule} from '@angular/material/icon';
import { throws } from 'assert';

interface Tile {
   x: number; 
   y: number;
   x_flatten: number;
   y_flatten: number;
   img_src: string;
   dice?: number;
}

interface Building {
  x: number;
  y: number;
  x_flatten: number;
  y_flatten: number;
  color: string,
  type: string,
  shift: boolean,
}


interface Street {
  b1_x_flatten: number,
  b1_y_flatten: number,
  b2_x_flatten: number,
  b2_y_flatten: number,

  x:number;
  y:number;
  rotation: number;
  length: number;
  heigth: number;
  color: string
}

@Component({
  selector: 'app-board',
  standalone: true,
  imports: [MatIconModule],
  templateUrl: './board.component.html',
  styleUrl: './board.component.scss'
})
export class BoardComponent implements OnInit {
  @Input() format: number[] = []
  @Input() scale: number = 0.7

  board_heigth = 0
  board_width = 0
  img_with = 175;
  img_heigt = 203;
  padding_x = 50;
  padding_y = 50;
  coordinats: Tile[] = []
  buildings: Building[] = []
  streets: Street[] = []

  constructor() {

  }


  
  ngOnInit(): void {
    //-------------------------------------------
    // Build tiles
    //-------------------------------------------

    for (let row = 0; row < this.format.length; row++) {
      let row_shift = Math.max(...this.format)  - this.format[row];
      for (let column = 0; column < this.format[row]; column++) {
        this.coordinats.push({
          x: (row_shift + column * 2) / 2 * this.img_with * this.scale + this.padding_x,
          y: row * (this.img_heigt * this.scale) - (row * this.img_heigt/3.8 * this.scale) + this.padding_y,
          img_src: "assets/images/sechseck.svg",
          //img_src: "assets/images/erz.png",
          dice: 6,
          x_flatten: column + row_shift,
          y_flatten: row
        })
      }
    }

    //-------------------------------------------
    // set board dimensions
    //-------------------------------------------


    this.board_heigth =(
      this.format.length * (this.img_heigt * this.scale) 
      - ((this.format.length-1) * this.img_heigt/3.8 * this.scale)
      + (this.padding_y*2)
    )

    this.board_width =(
      (Math.max(...this.format)  * 2) / 2 * this.img_with * this.scale + (this.padding_x*2)
    )
  
    //-------------------------------------------
    // Buildings
    //-------------------------------------------

    let max_row = Math.max(...this.format);
    let idx = this.format.indexOf(max_row);

    let updatet_fomat = this.format.slice(0, idx);
    updatet_fomat.push(max_row);
    updatet_fomat.push(...this.format.slice(idx))

    let shift_a = 40 * this.scale
    let shift_b = 0

    for (let row = 0; row < updatet_fomat.length; row++) {
      let row_shift = max_row  - updatet_fomat[row];

      if (row == idx+1){
        shift_a = 0
        shift_b = 40 * this.scale
      }

      let upper = true

      for (let i= 0; i < (updatet_fomat[row]*2) + 1; i++) {
        this.buildings.push({
          x: (row_shift + i) * (this.img_with/2) * this.scale + this.padding_x | 0,
          y: row * 152 * this.scale + this.padding_y + (upper? shift_a : shift_b) | 0,
          color: "#aaa",
          type: "cottage", // cottage, location_city
          x_flatten: i + row_shift,
          y_flatten: row,
          shift: upper
        })
        
        upper = !upper;
      }
    }


    //-------------------------------------------
    // Streets
    //-------------------------------------------


    console.log(this.buildings);
    


    for (let row = 0; row < updatet_fomat.length; row++) {
      let row_shift = (max_row  - updatet_fomat[row]);

      let root;
      if (row >= idx+1){
        root =  (a: Building, b: Building) => {return a.shift};
      }else {
        root =  (a: Building, b: Building) => {return b.shift};
      }


      for (let i= 0; i < (updatet_fomat[row]*2) ; i++) {

        let a = this.buildings.filter((b) => b.y_flatten == row && b.x_flatten == row_shift+i)[0]
        let b = this.buildings.filter((b) => b.y_flatten == row && b.x_flatten == row_shift+i+1)[0]

        let y_delta = Math.abs(a.y - b.y) / 2
        let y_root = Math.min(a.y, a.y)

        this.streets.push({
          b1_x_flatten: a.x_flatten,
          b1_y_flatten: a.y_flatten,
          b2_x_flatten: b.x_flatten,
          b2_y_flatten: b.y_flatten,
          color: "transparent",
          length: 30 * this.scale,
          heigth: 20 * this.scale,
          rotation: root(a,b)?30 : 150,
          x: Math.min(a.x, b.x) + Math.abs(a.x - b.x) / 2,
          y: root(a,b)? y_root + y_delta : y_root - y_delta,

        })

      }
    }

    console.log(this.streets)

    for (let row = 0; row < this.format.length; row++) {
      let row_shift = (max_row  - this.format[row]);
      for (let i= 0; i < this.format[row]+1 ; i++) {
        let c = row_shift + i*2;

        let a = this.buildings.filter((b) => b.y_flatten == row && b.x_flatten == c)[0]
        let b = this.buildings.filter((b) => b.y_flatten == row+1 && b.x_flatten == c)[0]


        this.streets.push({
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


    //-------------------------------------------
    // Ports
    //-------------------------------------------


  }
}
