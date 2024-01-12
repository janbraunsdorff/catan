import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DesignNewGameComponent } from './design-new-game.component';

describe('DesignNewGameComponent', () => {
  let component: DesignNewGameComponent;
  let fixture: ComponentFixture<DesignNewGameComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DesignNewGameComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DesignNewGameComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
