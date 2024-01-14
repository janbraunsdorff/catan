import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { BehaviorSubject, map } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class JoinGameService {

  games = new BehaviorSubject<Game[]>([])

  constructor(private http: HttpClient) { }


  get_games() {
    this.http.get<Game[]>(
      "http://127.0.0.1:3000/game/list",
    ).pipe(
      map( val => {
        const x = val.map(g => {return {...g, name:g.name.split(".")[0]}})
        return x
      })
    )
    .subscribe({
      next: data => this.games.next(data),
      error: err => console.log(err)
    })
  }
}


export interface Game {
  name: string,
  last_modified: string
}