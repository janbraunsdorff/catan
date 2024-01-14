import { Component, OnInit } from '@angular/core';
import {MatDividerModule} from '@angular/material/divider';
import {MatIconModule} from '@angular/material/icon';
import {MatListModule} from '@angular/material/list';
import {DatePipe} from '@angular/common';
import { Game, JoinGameService } from '../../service/server/join-game.service';



@Component({
  selector: 'app-join',
  standalone: true,
  imports: [MatDividerModule, MatIconModule, MatListModule, DatePipe],
  templateUrl: './join.component.html',
  styleUrl: './join.component.scss'
})
export class JoinComponent implements OnInit{
  games: Game[] =[]

  constructor(private joinService: JoinGameService){
    this.joinService.games.subscribe(val => this.games = val)

  }

  ngOnInit(): void {
    this.joinService.get_games()
  }
}
