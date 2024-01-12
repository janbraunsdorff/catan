import { Component, OnInit } from '@angular/core';
import {MatCardModule} from '@angular/material/card';
import {MatDividerModule} from '@angular/material/divider';
import { BoardService } from '../../service/board.service';
import { TileService } from '../../service/tile.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-board-desginer',
  standalone: true,
  imports: [MatCardModule, MatDividerModule],
  templateUrl: './board-desginer.component.html',
  styleUrl: './board-desginer.component.scss'
})
export class BoardDesginerComponent implements OnInit{
  private last_idx = ""

  constructor(public tileService: TileService,  private _route: ActivatedRoute) {
  }
  ngOnInit(): void {
    this._route.queryParams.subscribe((q) => {
      this.last_idx = q['tile']? q['tile']: ""
    })
  }
  
  update_dice(value: number) {
    this.tileService.set_dice_value(this.last_idx, value)
  }

  update_terra(value: string) {
    this.tileService.set_terra(this.last_idx, value)
  }


}
