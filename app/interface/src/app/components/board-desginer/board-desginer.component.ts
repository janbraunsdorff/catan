import { Component } from '@angular/core';
import {MatCardModule} from '@angular/material/card';
import {MatDividerModule} from '@angular/material/divider';
import { BoardService } from '../../service/board.service';

@Component({
  selector: 'app-board-desginer',
  standalone: true,
  imports: [MatCardModule, MatDividerModule],
  templateUrl: './board-desginer.component.html',
  styleUrl: './board-desginer.component.scss'
})
export class BoardDesginerComponent {
  private last_idx = ""

  constructor(public baord: BoardService) {
    baord.last_selected_tile.subscribe(e => this.last_idx = e)
  }
  
  update_dice(value: number) {
    this.baord.set_dice_value(this.last_idx, value)
  }

  update_terra(value: string) {
    this.baord.set_terra(this.last_idx, value)
  }


}
