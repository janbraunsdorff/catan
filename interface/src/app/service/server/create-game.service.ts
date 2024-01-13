import { Injectable } from '@angular/core';
import { TileService } from '../ui/tile.service';
import { PortsService } from '../ui/ports.service';
import { HttpClient } from '@angular/common/http';
import { BuildingService } from '../ui/building.service';


@Injectable({
  providedIn: 'root'
})
export class CreateGameService {

  constructor(private tileService: TileService, private portService: PortsService, private buildingService: BuildingService, private http: HttpClient) { }

  create_game(player: Player[], format: string) {
    if (Array.isArray(format)) {
      format = format[0]
    }
    let format_arr = format.split(",").map(x => +x)

    let t = this.tileService.tiles
    let p = this.portService.ports
    let b = this.buildingService.buildings;

    let request: CreateNewGameRequest =  {
      npc: player.filter(x => x.color !== "" && x.type === "npc"),
      player: player.filter(x => x.color !== "" && x.type === "pc"),
      extentiosns: []
    }

    let today = new Date();

    let error = false;

    this.http.post(
      "http://127.0.0.1:3000/game/" + today.toISOString() + "/create",
      request
    ).subscribe({
      next: data => console.log(data),
      error: err => {
        console.error(err) 
        error = true
      }
    })

    if (error) {
      return
    }

    let body: FillBoardRequest = {
      tiles: t.map(x => {
        let ttype = x.img_src.split("/").slice(-1)[0].split(".")[0];
        let b: TileRequest = {
          x: x.x_flatten, 
          y: x.y_flatten, 
          tile_type: ttype, 
          dice_value: x.dice!
        }; 
        return b
      }),
      format: format_arr,
      ports: p.map(x => {
        let p: PortRequest = {
          port_type: x.type,
          port_building: [
            {x: x.buildings[0].x, y:x.buildings[0].y},
            {x: x.buildings[1].x, y:x.buildings[1].y}
          ],
          flipped: x.flip
        }
        return p;
      }),
      buildings: b.map(x => {
        let b: BuildingRequest = {
          building_type: x.type,
          color: x.color,
          x: x.x_flatten,
          y: x.y_flatten
        }
        return b
      })
    }

    this.http.post(
      "http://127.0.0.1:3000/game/" + today.toISOString() + "/fill",
      body
    ).subscribe({
      next: data => console.log(data),
      error: err => {
        console.error(err) 
        error = true
      }
    })

  }
}


export interface Player {
  color: string;
  type: string;
  name: string
}

export interface CreateNewGameRequest {
  npc: Player[],
  player: Player[],
  extentiosns: string[]
}

export interface TileRequest {
  x: number,
  y: number,
  tile_type: string,
  dice_value: number
}

export interface PortRequest {
  port_type: string,
  port_building: {x: number, y: number}[],
  flipped: boolean
}

export interface BuildingRequest{
  building_type: string,
  color: string,
  x: number,
  y: number
}

export interface FillBoardRequest {
    tiles: TileRequest[],
    format: number[],
    ports: PortRequest[],
    buildings: BuildingRequest[]
}