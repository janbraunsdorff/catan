import { Component, EventEmitter, Output } from '@angular/core';
import {MatSelectModule} from '@angular/material/select';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatCheckboxModule} from '@angular/material/checkbox';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';


@Component({
  selector: 'app-player-select',
  standalone: true,
  imports: [MatSelectModule, MatFormFieldModule, MatCheckboxModule, FormsModule, ReactiveFormsModule],
  templateUrl: './player-select.component.html',
  styleUrl: './player-select.component.scss'
})
export class PlayerSelectComponent {
  @Output() player_type = new EventEmitter<string>();
  @Output() player_color = new EventEmitter<string>();
}
