import { Component, Input, OnInit, ɵɵpipeBind1 } from '@angular/core';
import { BoardService, Building, Street, Tile } from '../../service/board.service';
import { TileComponent } from "./tile/tile.component";
import { BuildingComponent } from "./building/building.component";
import { StreetComponent } from "./street/street.component";



@Component({
    selector: 'app-board',
    standalone: true,
    templateUrl: './board.component.html',
    styleUrl: './board.component.scss',
    imports: [TileComponent, BuildingComponent, StreetComponent]
})
export class BoardComponent implements OnInit {
  board_heigth = 0
  board_width = 0

  tiles: Tile[] = []
  buildings: Building[] = []
  streets: Street[] = []

  constructor(public baord: BoardService) {
    baord.tiles$.subscribe((s) => {this.tiles = s})
    baord.street$.subscribe((s) => {this.streets = s})
    baord.buildings$.subscribe((s) => {this.buildings = s})
    baord.board_heigth.subscribe(s => this.board_heigth = s)
    baord.board_width.subscribe(s => this.board_width = s)
    //baord.format.next([3,4,5,4,3])
  }
  ngOnInit(): void {

  }
}
