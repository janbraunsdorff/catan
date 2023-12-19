import { Component } from '@angular/core';
import {MatCardModule} from '@angular/material/card';
import {MatButtonModule} from '@angular/material/button';
import {FormBuilder, Validators, FormsModule, ReactiveFormsModule, FormControl} from '@angular/forms';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatStepperModule} from '@angular/material/stepper';

import {MatListModule} from '@angular/material/list';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';



@Component({
  selector: 'app-init-new-game',
  standalone: true,
  imports: [MatCardModule, MatButtonModule, MatButtonModule,
    MatStepperModule,
    FormsModule,
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatListModule, CommonModule, RouterModule],
  templateUrl: './init-new-game.component.html',
  styleUrl: './init-new-game.component.scss'
})
export class InitNewGameComponent {

  boardSize = this._formBuilder.group({
    size: new FormControl([Validators.required]),
    players: new FormControl([Validators.required]),
    npc: new FormControl([Validators.required])
  })

  constructor(private _formBuilder: FormBuilder) {
  }



}
