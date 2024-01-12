import { Injectable } from '@angular/core';
import { BehaviorSubject, Subject } from 'rxjs';
import { TileService } from './tile.service';
import { ConfigService } from './config.service';
import { Building, BuildingService } from './building.service';
import { StreetService } from './street.service';




@Injectable({
  providedIn: 'root'
})
export class BoardService {
  format = new Subject<number[]>();

  show_tiles = false;
  show_ports = false;
  show_buildings = false;
  show_streets = false;
  

  constructor(
    private buildingService: BuildingService, 
    private tileService: TileService, 
    private streetService: StreetService,
    private config: ConfigService) {
  }

  create() {
    this.format.subscribe((format) => {
      this.tileService.create_tiles(format)
      this.tileService.calcualate_size(format)
      let buildings = this.buildingService.create_buildings(format)
      this.streetService.create_streets(format, buildings)
    })
  }

}
