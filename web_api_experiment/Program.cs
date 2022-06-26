var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

var app = builder.Build();

// Configure the HTTP request pipeline.

app.MapGet("/easy", () =>
{
    var settings = new GameSettings(20, 40, "Blue");
    return settings;
});

app.MapGet("/hard", () =>
{
    var settings = new GameSettings(30, 60, "Red");
    return settings;
});

app.Run();

record GameSettings(int Rows, int Columns, string Color) { }