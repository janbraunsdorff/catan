import { Component, Input } from '@angular/core';
import { Building } from '../../../service/board.service';
import {MatIconModule} from '@angular/material/icon';


@Component({
  selector: 'app-building',
  standalone: true,
  imports: [MatIconModule],
  templateUrl: './building.component.html',
  styleUrl: './building.component.scss'
})
export class BuildingComponent {
  @Input() building: Building = {
    idx: "",
    x: 0,
    y: 0,
    x_flatten: 0,
    y_flatten: 0,
    color: '',
    type: '',
    shift: false
  }
}
