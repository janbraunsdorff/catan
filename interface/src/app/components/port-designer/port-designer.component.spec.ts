import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PortDesignerComponent } from './port-designer.component';

describe('PortDesignerComponent', () => {
  let component: PortDesignerComponent;
  let fixture: ComponentFixture<PortDesignerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PortDesignerComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PortDesignerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
