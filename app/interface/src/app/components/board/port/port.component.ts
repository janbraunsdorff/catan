import { Component, Input } from '@angular/core';
import {MatIconModule} from '@angular/material/icon';

@Component({
  selector: 'app-port',
  standalone: true,
  imports: [MatIconModule],
  templateUrl: './port.component.html',
  styleUrl: './port.component.scss'
})
export class PortComponent {
  @Input() size = 100;
  @Input() portType = "any" 
}
