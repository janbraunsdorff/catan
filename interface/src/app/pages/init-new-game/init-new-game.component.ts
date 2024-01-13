import { AfterViewInit, Component, ViewChild } from '@angular/core';
import {MatCardModule} from '@angular/material/card';
import {MatButtonModule} from '@angular/material/button';
import {FormBuilder, Validators, FormsModule, ReactiveFormsModule, FormControl} from '@angular/forms';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatStepper, MatStepperModule} from '@angular/material/stepper';

import {MatListModule} from '@angular/material/list';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { PlayerSelectComponent } from "../../components/player-select/player-select.component";
import { Player } from '../../service/server/create-game.service';



@Component({
    selector: 'app-init-new-game',
    standalone: true,
    templateUrl: './init-new-game.component.html',
    styleUrl: './init-new-game.component.scss',
    imports: [MatCardModule, MatButtonModule, MatButtonModule,
        MatStepperModule,
        FormsModule,
        ReactiveFormsModule,
        MatFormFieldModule,
        MatInputModule,
        MatListModule, CommonModule, RouterModule, PlayerSelectComponent]
})
export class InitNewGameComponent {
  format = []
  player: Player[] = [
    {color: '', type: '', name: ''},
    {color: '', type: '', name: ''},
    {color: '', type: '', name: ''},
    {color: '', type: '', name: ''},
  ]
  
  constructor(private _formBuilder: FormBuilder) {
  }
  
  set_color(idx: number, value: string) {
    this.player[idx].color = value
  }
  set_type(idx: number, value: string) {
    this.player[idx].type = value
  }
  
  json_player(): any {
    let s = JSON.stringify(this.player)
    return s
  }

}
