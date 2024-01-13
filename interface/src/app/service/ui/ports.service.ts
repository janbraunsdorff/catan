import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Building, BuildingService } from './building.service';
import { StreetService } from './street.service';
import { ConfigService } from './config.service';

export interface Port {

  buildings: Building[],
  street_idx: string;
  x: number,
  y: number,
  type: string,
  edit_mode: boolean,
  flip: boolean
}

@Injectable({
  providedIn: 'root'
})
export class PortsService {

  ports$ = new BehaviorSubject<Port[]>([]);
  ports: Port[] = [];



  constructor(private config: ConfigService, private buildingService: BuildingService, private streets: StreetService) { }

  create_port(street_idx: string, type: string) {
    let port = this.ports.filter(x => x.street_idx == street_idx)[0]
    if (port == undefined) {
      port = this.create_new_port(street_idx, type, false)
      this.ports.push(port)
    }else {
      port.type = type
    }

    this.update()
  }

  flip_port(street_idx: string, type: string) {
    let port = this.ports.filter(x => x.street_idx == street_idx)[0]
    let new_port = this.create_new_port(street_idx, type, !port.flip)
    port.x = new_port.x
    port.y = new_port.y
    port.flip = new_port.flip
    this.update()
  }

  delete_port(idx: string) {
    this.ports = this.ports.filter(e => e.street_idx != idx)
    this.update()
  }

  private update() {
    this.ports.push();
    this.ports$.next(this.ports)
  }

  private create_new_port(street_idx: string, type: string, flip: boolean): Port {
    let buildings = this.streets.streets.filter(x => x.idx == street_idx)[0].buildings
    let x_delta = buildings[0].x - buildings[1].x
    let y_delta = buildings[0].y - buildings[1].y

    let b1 = 100
    let b2 = ((x_delta * b1) * -1 ) / y_delta

    let x_middel = Math.min(buildings[0].x, buildings[1].x) + Math.abs( buildings[0].x - buildings[1].x) / 2
    let y_middel = Math.min(buildings[0].y, buildings[1].y) + Math.abs( buildings[0].y - buildings[1].y) /2

    let length = Math.sqrt(b1*b1 + b2*b2)
    let scale = 40 * this.config.scale

    b1 = b1 / length
    b2 = b2 / length
    
    b1 *= scale
    b2 *= scale

    let x = 0;
    let y = 0;

    if (flip) {
      x = b1 + x_middel
      y = b2 + y_middel
    }else {
      x = x_middel - b1
      y = y_middel - b2
    }

    let port =  {
      buildings: buildings,
      street_idx: street_idx,
      type: type,
      edit_mode: false,
      x: x,
      y: y,
      flip: flip
    }
    return port
  }
}
