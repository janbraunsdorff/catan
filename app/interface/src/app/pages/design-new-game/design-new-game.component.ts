import { AfterViewInit, Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import {MatTabChangeEvent, MatTabsModule} from '@angular/material/tabs';
import { BoardComponent } from "../../components/board/board.component";
import { BoardService } from '../../service/board.service';
import { BoardDesginerComponent } from "../../components/board-desginer/board-desginer.component";
import { PortDesignerComponent } from '../../components/port-designer/port-designer.component';

@Component({
    selector: 'app-design-new-game',
    standalone: true,
    templateUrl: './design-new-game.component.html',
    styleUrl: './design-new-game.component.scss',
    imports: [BoardComponent, BoardDesginerComponent, MatTabsModule, PortDesignerComponent]
})
export class DesignNewGameComponent implements OnInit {
  current_index = 0
  board_size = []
  constructor(private route: ActivatedRoute, public baord: BoardService,  private router: Router) {
  }

  ngOnInit(): void {
    this.route.queryParams.subscribe((q) => {
      let size: string = q['size']
      if (Array.isArray(size)) {
        size = size[0]
      }
      let board_size = size.split(",").map((x) => +x )
      this.baord.create()
      this.baord.format.next(board_size)


      if (q['mode'] == undefined) {
        this.router.navigate([], {
          relativeTo: this.route,
          queryParams: {
            mode: 'Tiles'
          },
          queryParamsHandling: 'merge',
          skipLocationChange: false
        });
      }else if (q['mode'] === "Tiles") {
        this.display_board()
        this.current_index = 0
      }else if (q['mode'] === "Ports") {
        this.display_port()
        this.current_index = 1
      }

    })
  }


  tabChanged(e: MatTabChangeEvent) {
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: {
        mode: e.tab.textLabel,
        street: null,
        tile: null
      },
      queryParamsHandling: 'merge',
      skipLocationChange: false
    });
  }

  private display_port() {
    this.baord.show_tiles = true;
    this.baord.show_ports = true;
    this.baord.show_buildings = false;
    this.baord.show_streets = true;
  }

  private display_board() {
    this.baord.show_tiles = true;
    this.baord.show_ports = true;
    this.baord.show_buildings = false;
    this.baord.show_streets = false;
  }
}
