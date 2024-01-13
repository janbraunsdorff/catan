import { AfterViewChecked, AfterViewInit, Component, Input, OnInit } from '@angular/core';
import { BoardService } from '../../../service/ui/board.service';
import { ActivatedRoute, Router } from '@angular/router';
import { Street } from '../../../service/ui/street.service';

@Component({
  selector: 'app-street',
  standalone: true,
  imports: [],
  templateUrl: './street.component.html',
  styleUrl: './street.component.scss'
})
export class StreetComponent implements OnInit {

  @Input() s: Street = {
    idx: '',
    buildings: [],
    x: 0,
    y: 0,
    rotation: 0,
    length: 0,
    heigth: 0,
    color: '',
    edit_mode: false
  }

  constructor(public baord: BoardService, private _route: ActivatedRoute,
    private _router: Router) { }


  ngOnInit(): void {
    this._route.queryParams.subscribe((q) => {
      this.s.edit_mode = q['street'] == this.s.idx
    })
  }


  activate() {
    this._router.navigate([], {
      relativeTo: this._route,
      queryParams: {
        street: this.s.idx
      },
      queryParamsHandling: 'merge',
      skipLocationChange: false
    });
  }

}
