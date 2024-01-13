import { Component, Input, OnDestroy, OnInit, ɵɵpipeBind1 } from '@angular/core';
import { BoardService } from '../../service/ui/board.service';
import { TileComponent } from "./tile/tile.component";
import { BuildingComponent } from "./building/building.component";
import { StreetComponent } from "./street/street.component";
import { Tile, TileService } from '../../service/ui/tile.service';
import { ConfigService } from '../../service/ui/config.service';
import { Street, StreetService } from '../../service/ui/street.service';
import { Building, BuildingService } from '../../service/ui/building.service';
import { Port, PortsService } from '../../service/ui/ports.service';
import { PortComponent } from "./port/port.component";



@Component({
    selector: 'app-board',
    standalone: true,
    templateUrl: './board.component.html',
    styleUrl: './board.component.scss',
    imports: [TileComponent, BuildingComponent, StreetComponent, PortComponent]
})
export class BoardComponent implements OnInit, OnDestroy {
  board_heigth = 0
  board_width = 0

  tiles: Tile[] = []
  buildings: Building[] = []
  streets: Street[] = []
  ports: Port[] = []

  constructor(
    public baord: BoardService, 
    public tileService: TileService, 
    public config: ConfigService,
    private street: StreetService,
    public building: BuildingService,
    public port: PortsService
  ) {

  }
  ngOnDestroy(): void {
    this.tileService.tiles$.unsubscribe()
    this.street.street$.unsubscribe()
    this.building.buildings$.unsubscribe()
    this.config.board_heigth.unsubscribe()
    this.config.board_width.unsubscribe()
    this.port.ports$.unsubscribe()
  }
  ngOnInit(): void {
    this.tileService.tiles$.subscribe((s) => {this.tiles = s})
    this.street.street$.subscribe((s) => {this.streets = s})
    this.building.buildings$.subscribe((s) => {this.buildings = s})
    this.config.board_heigth.subscribe(s => this.board_heigth = s)
    this.config.board_width.subscribe(s => this.board_width = s)
    this.port.ports$.subscribe((s) => this.ports = s)
  }
}
