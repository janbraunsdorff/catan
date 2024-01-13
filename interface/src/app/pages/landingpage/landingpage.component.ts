import { Component } from '@angular/core';
import {MatIconModule} from '@angular/material/icon';
import {RouterModule} from '@angular/router';


@Component({
  selector: 'app-landingpage',
  standalone: true,
  imports: [MatIconModule, RouterModule],
  templateUrl: './landingpage.component.html',
  styleUrl: './landingpage.component.scss'
})
export class LandingpageComponent {

}
