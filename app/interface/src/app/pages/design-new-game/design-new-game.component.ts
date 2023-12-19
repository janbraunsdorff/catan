import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { BoardComponent } from "../../components/board/board.component";
import { BoardService } from '../../service/board.service';
import { BoardDesginerComponent } from "../../components/board-desginer/board-desginer.component";


@Component({
    selector: 'app-design-new-game',
    standalone: true,
    templateUrl: './design-new-game.component.html',
    styleUrl: './design-new-game.component.scss',
    imports: [BoardComponent, BoardDesginerComponent]
})
export class DesignNewGameComponent implements OnInit {
  board_size = []
  constructor(private route: ActivatedRoute, public baord: BoardService) {
    baord.set_mode_design()
  }

  ngOnInit(): void {
    this.route.queryParams.subscribe((q) => {
      let size: string = q['size']
      if (Array.isArray(size)) {
        size = size[0]
      }
      console.log(size)
      let board_size = size.split(",").map((x) => +x )
      this.baord.create_tiles_only()
      this.baord.format.next(board_size)
    })
  }
}
