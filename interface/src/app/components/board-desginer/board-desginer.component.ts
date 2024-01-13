import { Component, OnInit } from '@angular/core';
import {MatCardModule} from '@angular/material/card';
import {MatDividerModule} from '@angular/material/divider';
import { BoardService } from '../../service/ui/board.service';
import { TileService } from '../../service/ui/tile.service';
import { ActivatedRoute, UrlSerializer } from '@angular/router';
import {MatButtonModule} from '@angular/material/button';
import { CreateGameService, Player } from '../../service/server/create-game.service';


@Component({
  selector: 'app-board-desginer',
  standalone: true,
  imports: [MatCardModule, MatDividerModule, MatButtonModule],
  templateUrl: './board-desginer.component.html',
  styleUrl: './board-desginer.component.scss'
})
export class BoardDesginerComponent implements OnInit{
  private last_idx = ""
  format = ""
  player: Player[] = []

  constructor(public tileService: TileService,  private _route: ActivatedRoute, public create_Game: CreateGameService,  private serializer: UrlSerializer) {
  }
  ngOnInit(): void {
    this._route.queryParams.subscribe((q) => {
      this.last_idx = q['tile']? q['tile']: ""
      this.format = q['size']? q['size']: []
      this.player = q['player']? JSON.parse(q['player']!) as Player[] : []
    })
  }
  
  update_dice(value: number) {
    this.tileService.set_dice_value(this.last_idx, value)
  }

  update_terra(value: string) {
    this.tileService.set_terra(this.last_idx, value)
  }


}
