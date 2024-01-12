import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ConfigService {

  board_heigth = new BehaviorSubject(0);
  board_width = new BehaviorSubject(0);

  img_with = 175;
  img_heigt = 203;
  padding_x = 50;
  padding_y = 50;


  scale = 0.5

  constructor() { }
}
