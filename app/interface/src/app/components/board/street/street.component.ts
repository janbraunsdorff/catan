import { Component, Input } from '@angular/core';
import { Street } from '../../../service/board.service';

@Component({
  selector: 'app-street',
  standalone: true,
  imports: [],
  templateUrl: './street.component.html',
  styleUrl: './street.component.scss'
})
export class StreetComponent {
  @Input() s: Street ={
    idx: '',
    b1_x_flatten: 0,
    b1_y_flatten: 0,
    b2_x_flatten: 0,
    b2_y_flatten: 0,
    x: 0,
    y: 0,
    rotation: 0,
    length: 0,
    heigth: 0,
    color: ''
  }

}
