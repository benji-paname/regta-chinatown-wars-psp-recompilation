param([Parameter(Mandatory=$true)][string]$Svg,[Parameter(Mandatory=$true)][string]$Png,[int]$Scale=2)
Add-Type -AssemblyName PresentationCore
Add-Type -AssemblyName WindowsBase
[xml]$document=Get-Content -LiteralPath $Svg -Raw
$width=[double]$document.svg.width
$height=[double]$document.svg.height
$visual=New-Object System.Windows.Media.DrawingVisual
$context=$visual.RenderOpen()
$context.PushTransform((New-Object System.Windows.Media.ScaleTransform $Scale,$Scale))
foreach ($element in $document.svg.ChildNodes) {
    if ($element.LocalName -eq 'rect') {
        $brush=New-Object System.Windows.Media.SolidColorBrush ([System.Windows.Media.ColorConverter]::ConvertFromString($element.fill))
        $context.DrawRectangle($brush,$null,(New-Object System.Windows.Rect 0,0,$width,$height))
    }
    if ($element.LocalName -eq 'path') {
        $geometry=[System.Windows.Media.Geometry]::Parse($element.d)
        $brush=New-Object System.Windows.Media.SolidColorBrush ([System.Windows.Media.ColorConverter]::ConvertFromString($element.fill))
        $pen=$null
        if ($element.stroke) {
            $stroke=New-Object System.Windows.Media.SolidColorBrush ([System.Windows.Media.ColorConverter]::ConvertFromString($element.stroke))
            $pen=New-Object System.Windows.Media.Pen $stroke,([double]$element.'stroke-width')
            $pen.LineJoin=[System.Windows.Media.PenLineJoin]::Round
        }
        $translated=$false
        if ($element.transform -match 'translate\(([-.\d]+)[ ,]+([-.\d]+)\)') {
            $context.PushTransform((New-Object System.Windows.Media.TranslateTransform ([double]$Matches[1]),([double]$Matches[2])))
            $translated=$true
        }
        if ($element.'paint-order' -eq 'stroke fill') {$context.DrawGeometry($null,$pen,$geometry);$context.DrawGeometry($brush,$null,$geometry)}
        else {$context.DrawGeometry($brush,$pen,$geometry)}
        if ($translated) {$context.Pop()}
    }
    if ($element.LocalName -eq 'text') {
        $typeface=New-Object System.Windows.Media.Typeface 'Arial'
        $brush=New-Object System.Windows.Media.SolidColorBrush ([System.Windows.Media.ColorConverter]::ConvertFromString($element.fill))
        $chars=@();$total=0.0;$gap=[double]$element.'letter-spacing'
        foreach ($char in $element.InnerText.ToCharArray()) {
            $text=New-Object System.Windows.Media.FormattedText ([string]$char),([Globalization.CultureInfo]::InvariantCulture),([System.Windows.FlowDirection]::LeftToRight),$typeface,([double]$element.'font-size'),$brush,1.0
            $chars+=$text;$total+=$text.WidthIncludingTrailingWhitespace+$gap
        }
        $x=[double]$element.x-($total-$gap)/2
        foreach ($text in $chars) {$context.DrawText($text,(New-Object System.Windows.Point $x,([double]$element.y-$text.Baseline)));$x+=$text.WidthIncludingTrailingWhitespace+$gap}
    }
}
$context.Pop();$context.Close()
$bitmap=New-Object System.Windows.Media.Imaging.RenderTargetBitmap ([int]($width*$Scale)),([int]($height*$Scale)),96,96,([System.Windows.Media.PixelFormats]::Pbgra32)
$bitmap.Render($visual)
$encoder=New-Object System.Windows.Media.Imaging.PngBitmapEncoder
$encoder.Frames.Add([System.Windows.Media.Imaging.BitmapFrame]::Create($bitmap))
$stream=[IO.File]::Create($Png)
try {$encoder.Save($stream)} finally {$stream.Dispose()}
Write-Output $Png
