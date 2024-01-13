import { Routes } from '@angular/router';
import { BoardComponent } from './components/board/board.component';
import { InitNewGameComponent } from './pages/init-new-game/init-new-game.component';
import { DesignNewGameComponent } from './pages/design-new-game/design-new-game.component';
import { LandingpageComponent } from './pages/landingpage/landingpage.component';
import { JoinComponent } from './pages/join/join.component';

export const routes: Routes = [
    {path: '', pathMatch: 'full', component: LandingpageComponent},
    {path: 'join', component: JoinComponent},
    {path: 'new/init', component: InitNewGameComponent},
    {path: 'new/design', component: DesignNewGameComponent}
];
