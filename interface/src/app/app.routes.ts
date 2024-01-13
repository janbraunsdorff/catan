import { Routes } from '@angular/router';
import { BoardComponent } from './components/board/board.component';
import { InitNewGameComponent } from './pages/init-new-game/init-new-game.component';
import { DesignNewGameComponent } from './pages/design-new-game/design-new-game.component';
import { LandingpageComponent } from './pages/landingpage/landingpage.component';

export const routes: Routes = [
    {path: '', pathMatch: 'full', component: LandingpageComponent},
    {path: 'new/init', component: InitNewGameComponent},
    {path: 'new/design', component: DesignNewGameComponent}
];
