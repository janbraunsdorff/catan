import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BoardDesginerComponent } from './board-desginer.component';

describe('BoardDesginerComponent', () => {
  let component: BoardDesginerComponent;
  let fixture: ComponentFixture<BoardDesginerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BoardDesginerComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(BoardDesginerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
