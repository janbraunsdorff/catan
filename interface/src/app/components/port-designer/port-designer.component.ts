import { Component, OnInit } from '@angular/core';
import {MatCardModule} from '@angular/material/card';
import {MatDividerModule} from '@angular/material/divider';
import { PortComponent } from "../board/port/port.component";
import { ActivatedRoute, Router } from '@angular/router';
import { Port, PortsService } from '../../service/ui/ports.service';
import {MatButtonModule} from '@angular/material/button';



@Component({
    selector: 'app-port-designer',
    standalone: true,
    templateUrl: './port-designer.component.html',
    styleUrl: './port-designer.component.scss',
    imports: [MatCardModule, MatDividerModule, PortComponent, MatButtonModule]
})
export class PortDesignerComponent implements OnInit{
  constructor(private _route: ActivatedRoute,
    private _router: Router, public portService: PortsService) { }

  size = 75;
  ports: Port[] = []
  private current_street = ""

  ngOnInit(): void {
    this._route.queryParams.subscribe((q) => {
        this.current_street = q['street']
    })

    this.portService.ports$.subscribe(s => this.ports = s)
  }

  add_port(type: string) {
    this.portService.create_port(this.current_street, type)
  }


  select(street_idx: string) {
    this._router.navigate([], {
      relativeTo: this._route,
      queryParams: {
        street: street_idx
      },
      queryParamsHandling: 'merge',
      skipLocationChange: false
    });
  }
}
