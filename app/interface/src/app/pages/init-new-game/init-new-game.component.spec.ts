import { ComponentFixture, TestBed } from '@angular/core/testing';

import { InitNewGameComponent } from './init-new-game.component';

describe('InitNewGameComponent', () => {
  let component: InitNewGameComponent;
  let fixture: ComponentFixture<InitNewGameComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [InitNewGameComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(InitNewGameComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
