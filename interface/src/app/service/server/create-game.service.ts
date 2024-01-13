import { Injectable } from '@angular/core';
import { TileService } from '../ui/tile.service';
import { PortsService } from '../ui/ports.service';
import { HttpClient } from '@angular/common/http';


@Injectable({
  providedIn: 'root'
})
export class CreateGameService {

  constructor(private tileService: TileService, private portService: PortsService, private http: HttpClient) { }

  create_game(player: Player[]) {
    let t = this.tileService.tiles
    let p = this.portService.ports

    let request: CreateNewGameRequest =  {
      npc: player.filter(x => x.color !== "" && x.type === "npc"),
      player: player.filter(x => x.color !== "" && x.type === "pc"),
      extentiosns: []
    }

    let today = new Date();

    console.log(JSON.stringify(request))

    this.http.post(
      "http://127.0.0.1:3000/game/" + today.toISOString() + "/create",
      request
    ).subscribe({
      next: data => console.log(data),
      error: err => console.log(err) 
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